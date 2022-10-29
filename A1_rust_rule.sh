#!/bin/bash

CURPATH=$(pwd)
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" 

cd $HOME/zbin/win_rbin/A1_rust_rule

$HOME/.cargo/bin/cargo build

if [$? -ne 0 ];then
    echo "_____________ cargo build failed _____________ please check ! "
else
    echo "_____________ cargo build success _____________  ! "
$HOME/zbin/win_rbin/A1_rust_rule/target/debug/A1_rust_rule  $CURPATH %1 %2 %3 %4 %5 %6 %7 %8 %9
fi

cd $CURPATH