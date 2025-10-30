#!/bin/bash

if [ "$#" -ne 2 ]
then
     >&2 echo "Usage: ${0} <input> <output>"
    exit 1
fi

SOURCE="${BASH_SOURCE[0]}"
while [ -h "$SOURCE" ]; do # resolve $SOURCE until the file is no longer a symlink
  DIR="$( cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd )"
  SOURCE="$(readlink "$SOURCE")"
  [[ $SOURCE != /* ]] && SOURCE="$DIR/$SOURCE" # if $SOURCE was a relative symlink, we need to resolve it relative to the path where the symlink file was located
done
DIR="$( cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd )"

test="(cd ../../../../; cargo run -p cmemu-flash-test -- --skip-integrity-check --show-dump)" 

tzst=`readlink -f "${1}"`
output=`readlink -f "${2}"`
tmp=`readlink -f /tmp/.data_parser.output`

cd "${DIR}/../../../.."
for i in `seq 0 100000`
do
	cargo run -p cmemu-flash-test -- --skip-integrity-check --show-dump "${tzst}.${i}" >"${tmp}.${i}" 2>/dev/null
	exit_code="${?}"
	if ((exit_code == 1))
	then
		break
	fi
	echo "Done from ${1}.${i} to ${2}.${i}"
	if ((exit_code != 101))
	then
		echo "Bad exit code"
		echo "${i} ${exit_code}"
		exit 1
	fi
	"${DIR}/data_parser.py" <"${tmp}.${i}" >"${output}.${i}"
	rm -f "${tmp}.${i}"
done
