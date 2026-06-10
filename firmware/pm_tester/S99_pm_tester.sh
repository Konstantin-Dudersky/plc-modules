#!/bin/bash

cd /root

case "$1" in
	start)
		./pm_tester &
	;;
	stop)
		killall -q pm_tester
	;;
	*)
		echo "Usage: <servicename> {start|stop|status|reload|restart[|probe]"
		exit 1
	;;
esac
