set -e
for i in {1..10}; do
  tindersort names.txt -s ${i}${$} -f -p | dmenu -l 2 -p "who is better?" | tindersort names.txt -s ${i}${$} -f
done
