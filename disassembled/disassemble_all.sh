if [ $# -ne 0 ] ; then
  cargo run --release --bin disassembler -- $1 > "$(dirname $0)/$(basename $1 .umz).asm"
  exit 0
fi

echo "No arguments provided, disassemling all *.umz"

for f in $(dirname $0)/../files/*.umz ; do
  cargo run --release --bin disassembler -- $f > "$(dirname $0)/$(basename $f .umz).asm"
done
