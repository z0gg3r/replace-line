#!/bin/sh
# SPDX-FileCopyRightText: 2022 zocker <zocker@10zen.eu>
# SPDX-License-Identifier: GPL-3.0-or-later

print_usage()
{
	printf "replace-line "
	printf "\033[32;48mFILE\033[0m "
	printf "\033[33;48mPATTERN\033[0m "
	printf "[\033[35;48mREPLACEMENT\033[0m]"
	printf "\n"
}

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

if [ "$#" -lt 2 ]
then
	print_usage
	exit 1
elif [ -e "$1" ]
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
