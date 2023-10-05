address=~/Run/language_generator/backend/src/
address=~/Run/angelspeech/src/

cd "${address}bak"

for file in *.bak; do
    new_name="${file%.bak}.json"
    cp "$file" "${address}settings/${new_name}"
done
