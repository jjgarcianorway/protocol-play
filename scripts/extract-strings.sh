#!/usr/bin/env bash
# Extract all dialog strings from Rust source into a JSON translation template.
# Usage: ./scripts/extract-strings.sh > assets/i18n/template.json
#
# The output is a flat JSON object where keys are "scene_id.node_index"
# and values are the English text. Translators copy this file, rename it
# (e.g. es.json), and replace the values with translated text.

set -euo pipefail

SCENE_DIR="src/mission"

echo "{"

first=true

# Process each dialog scene file
for f in "$SCENE_DIR"/dialog_scenes_*.rs; do
    current_scene=""
    current_node=-1
    in_choices=false
    choice_idx=0

    while IFS= read -r line; do
        # Detect scene ID
        if [[ "$line" =~ id:\ \"([^\"]+)\" ]]; then
            current_scene="${BASH_REMATCH[1]}"
            current_node=-1
            in_choices=false
        fi

        # Detect node index from comments like "// 0", "// 1", etc.
        if [[ "$line" =~ ^[[:space:]]*//[[:space:]]*([0-9]+) ]]; then
            current_node="${BASH_REMATCH[1]}"
            in_choices=false
        fi

        # Detect DialogNode text
        if [[ -n "$current_scene" && "$current_node" -ge 0 ]]; then
            if [[ "$line" =~ text:\ \"(.*)\" ]]; then
                text="${BASH_REMATCH[1]}"
                # Handle continuation lines (backslash)
                text="${text%\\}"
                key="${current_scene}.${current_node}"
                if [ "$first" = true ]; then
                    first=false
                else
                    echo ","
                fi
                # Escape for JSON
                escaped=$(echo -n "$text" | sed 's/\\/\\\\/g; s/"/\\"/g')
                printf '  "%s": "%s"' "$key" "$escaped"
            fi
        fi

        # Detect choice text
        if [[ "$line" =~ DialogChoice\ \{\ text:\ \"(.*)\" ]]; then
            choice_text="${BASH_REMATCH[1]}"
            choice_text="${choice_text%\\}"
            if [[ -n "$current_scene" && "$current_node" -ge 0 ]]; then
                key="${current_scene}.${current_node}.choice.${choice_idx}"
                echo ","
                escaped=$(echo -n "$choice_text" | sed 's/\\/\\\\/g; s/"/\\"/g')
                printf '  "%s": "%s"' "$key" "$escaped"
                choice_idx=$((choice_idx + 1))
            fi
        fi

        # Reset choice counter when entering new choices block
        if [[ "$line" =~ DialogNext::Choice ]]; then
            choice_idx=0
        fi

    done < "$f"
done

echo ""
echo "}"
