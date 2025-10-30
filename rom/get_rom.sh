#!/bin/sh
# Note: use shellcheck to prevent bashism
# In CI, you can put all the files at $CACHE or pass --yes
# Or, even simpler, you can call this script once from $CACHE with --yes, and then do cheap calls in this dir.

HERE=$(dirname $0)
CACHE=${CACHE-$HOME}

check_candidate() {
  [ -r "$2" ] && echo "$(grep "$1" "$HERE/checksums.md5" | head -c 32)" "*$2" | md5sum -c --status -
}

find_symlink_in_home() {
  check_candidate "$1" "$CACHE/$1" && ln -s "$CACHE/$1" "$1"
}

# quick path
[ -r driverlib.elf ] || find_symlink_in_home driverlib.elf
[ -r driverlib.bin ] || find_symlink_in_home driverlib.bin
[ -r driverlib.c ] || find_symlink_in_home driverlib.c
[ -r brom.bin ] || find_symlink_in_home brom.bin

if [ ! -r driverlib.elf ]; then
  # Linking to Contiki, or find another from
  # https://github.com/search?q=path%3A**cc26x0%2From%2Fdriverlib.c&type=code
  elf="https://github.com/contiki-ng/coresdk_cc13xx_cc26xx/raw/refs/heads/master/source/ti/devices/cc26x0/rom/driverlib.elf"
  src="https://github.com/contiki-ng/coresdk_cc13xx_cc26xx/raw/refs/heads/master/source/ti/devices/cc26x0/rom/driverlib.c"

  # Go find yourself the license and the SDK download link at https://dev.ti.com/tirex/
  # It's probably the same as in the new GitHub SDK release:
  # https://github.com/TexasInstruments/simplelink-lowpower-f2-sdk?tab=License-1-ov-file
  # But cc2650 is already dropped there
  if [ "$1" != "--yes" ]; then
    printf "Have you read and accept the TI SimpleLink SDK license? (y/n)? "
    read -r ok
    [ "$ok" = "y" ] || exit
  fi
  if [ "$(command -v wget)" ]; then
    wget -q --show-progress "$elf" "$src"
  elif [ "$(command -v curl)" ]; then
    curl -L --remote-name-all "$elf" "$src"
  fi
fi
if [ ! -r driverlib.bin ]; then
  if [ "$(command -v arm-none-eabi-objcopy)" ]; then
    arm-none-eabi-objcopy -O binary -g -R STACK_SPACE -R RAM_CODE driverlib.elf driverlib.bin
  else
    objcopy -I elf32-little -O binary -g -R STACK_SPACE -R RAM_CODE driverlib.elf driverlib.bin
  fi
  chmod -x driverlib.bin # ?!
fi

if [ ! -r brom.bin ]; then
  playground_dump=$HERE/../../../mm319369/mem_mapping/dump_hwtest/dump_radioserver.tgz
  if [ -r $playground_dump ]; then
    tar xf "$playground_dump" dump_radioserver/dump_10000000_1001cc00.bin --xform 's/.*/brom.bin/'
  else
    echo "The full brom.bin is missing."
    echo "Check if you can access our internal wiki <https://github.com/mimuw-distributed-systems-group/cmemu-meta/wiki>."
    echo "Otherwise, follow steps from README or contact the team if you really need it."
    exit 1
  fi
fi

exec md5sum -c "$HERE/checksums.md5"
