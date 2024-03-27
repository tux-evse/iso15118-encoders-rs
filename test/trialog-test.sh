#!/bin/sh

if test -z "$TRIALOG_IP"; then
 TRIALOG_IP=trialog-ipv4
fi

if test -z "$1"; then
 TIMEOUT=60
fi

ping -c 1 $TRIALOG_IP -q > /dev/null
if test $? -ne $?; then
    echo "fail to contact trialog host:[$TRIALOG_IP]"
    exit 1
fi

trap ctrl_c_cb INT
function ctrl_c_cb {
    echo "Post http://$TRIALOG_IP:15110/api/plugout"
    curl -X POST http://$TRIALOG_IP:15110/api/plugout
}

# force reset in case combo was not properly disconnected
curl -X POST http://trialog-ipv4:15110/api/plugout
curl -X POST http://trialog-ipv4:15110/api/plugin
if test $? -ne $?; then
    echo "fail to post: api/plugin to combo:[$TRIALOG_IP]"
    exit 1
fi


echo "Use control-c to unplug from trialog (auto timeout:$TIMEOUT)"
sleep $TIMEOUT && curl -X POST http://$TRIALOG_IP:15110/api/plugout