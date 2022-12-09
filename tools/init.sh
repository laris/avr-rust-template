#!/usr/bin/env sh
set -e 
#set -ex 

# ./init.sh try6  m328p l
# ./init.sh $name $chip l=local

CRATE_NAME="$1"
AVR_TARGET="$2"
TEMPLATE="$3"
#TEMPLATE_LOCAL="$HOME/codes/avr-rust/avr_rust_crates/template-bin_laris"
TEMPLATE_LOCAL="$HOME/coding/rust-embedded/avr-rust_template-bin_laris_2021-09-25"
#TEMPLATE_URL="https://codeload.github.com/avr-rust/template-bin/tar.gz/master"
TEMPLATE_REMOTE="https://codeload.github.com/laris/template-bin/tar.gz/master"

if [ $# -eq 0 ]; then
  #echo "$0 crate_name avr_target [template optional <l=local>, <o=online>, </local/template/path>]"
  echo "$0 crate_name <avr_target=m328p local_template_bin>"

elif [ $# -eq 1 ]; then
  # default arg -> CRATE_NAME="$1"
  # default target m328p
  AVR_TARGET="atmega328p"
  echo "init crate name = \"$CRATE_NAME\" for avr target = \"$AVR_TARGET\" with local template-bin"
  # default template-bin in local
  cp -r $TEMPLATE_LOCAL $CRATE_NAME
  # set crate name
  gsed -i "s/template-bin/$CRATE_NAME/g" $CRATE_NAME/Cargo.toml
  # clear git config and re-init
  rm -rf $CRATE_NAME/.git
  cd $CRATE_NAME
  git init

# elif [ $# -eq 2 ]; then
#   echo "init crate name = \"$CRATE_NAME\" for avr target = \"$AVR_TARGET\" with online template-bin"
#   # Download and extract the template repository to the current directory.
#   curl $TEMPLATE_ONLINE_URL | tar xzvf -
#   gsed -i "s/template-bin/$CRATE_NAME/g" template-bin-master/Cargo.toml
#   # Then update the package name in Cargo.toml
#   mv template-bin-master $CRATE_NAME
#   # Prepare a new Git repository
#   cd $CRATE_NAME
#   git init
# 
# elif [ $# -eq 3 ]; then
#   if   [[ $AVR_TARGET = "m328p" ]] || [[ $AVR_TARGET = "atmega328p" ]]; then
#     AVR_TARGET="atmega328p"
#   elif [[ $AVR_TARGET = "m1280" ]] || [[ $AVR_TARGET = "atmega1280" ]]; then
#     AVR_TARGET="atmega1280"
#   elif [[ $AVR_TARGET = "m2560" ]] || [[ $AVR_TARGET = "atmega2560" ]]; then
#     AVR_TARGET="atmega2560"
#   elif [[ $AVR_TARGET = "m32u4" ]] || [[ $AVR_TARGET = "atmega32u4" ]]; then
#     AVR_TARGET="atmega32u4"
#   elif [[ $AVR_TARGET = "tn85" ]] || [[ $AVR_TARGET = "attiny85" ]]; then
#     AVR_TARGET="attiny85"
#   elif [[ $AVR_TARGET = "tn86" ]] || [[ $AVR_TARGET = "attiny86" ]]; then
#     AVR_TARGET="attiny86"
#   else
#     echo "customer chip code"
#   fi
# 
#   if [[ $TEMPLATE_LOCAL_PATH = "l" ]]; then
#     # copy local path
#     echo "init crate name = \"$CRATE_NAME\" for avr target = \"$AVR_TARGET\" with local template \"$TEMPLATE_LOCAL\""
#     cp -r $TEMPLATE_LOCAL $CRATE_NAME
#   elif [[ $TEMPLATE_LOCAL_PATH = "o" ]]; then
#     echo "init crate name = \"$CRATE_NAME\" for avr target = \"$AVR_TARGET\" with online template \"$TEMPLATE_ONLINE_URL\""
#     curl $TEMPLATE_ONLINE_URL | tar xzvf -
#     gsed -i "s/template-bin/$CRATE_NAME/g" template-bin-master/Cargo.toml
#     mv template-bin-master $CRATE_NAME
#   else
#     echo "init crate name = \"$CRATE_NAME\" for avr target = \"$AVR_TARGET\" with local template \"$TEMPLATE_LOCAL_PATH\""
#     cp -r $TEMPLATE_LOCAL_PATH $CRATE_NAME
#     cd $CRATE_NAME
#     git init
#   fi
# 
#   gsed -i "s/template-bin/$CRATE_NAME/g" $CRATE_NAME/Cargo.toml
#   rm -rf $CRATE_NAME/.git
#   cd $CRATE_NAME
#   git init

else 
  echo "$0 crate_name avr_target=m328p local_template_bin"
  #echo "$0 crate_name avr_target [template optional <l=local>, <o=online>, </local/template/path>]"
fi
