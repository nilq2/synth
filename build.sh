while inotifywait -q -r -e modify ./ ../examples/
do
   clear
   clear
   cargo run
done
