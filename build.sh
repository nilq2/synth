while inotifywait -q -r -e close_write ./ ../examples/
do
   clear
   clear
   cargo run
done
