#! /bin/sh

replace_line()
{
	if [ -z "$1" ]
	then
		true
	else
		echo "$1"
	fi
}

die()
{
	echo "$1"
	exit 1
}

if [ -e "$1" ]
then
	cat "$1" | while read -r line
	do
		case $line in
			*$2*) replace_line "$3" ;;
			*) echo "$line" ;;
		esac
	done
else
	die "$1 does not exist."
fi
