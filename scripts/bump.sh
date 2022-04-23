#!/bin/zsh

old_full_version=$(cat Cargo.toml | grep "version" | cut -f 3 -d " " | grep -Po '[0-9]\.[0-9]\.[0-9]')
version=($(echo $old_full_version | tr "." "\n"))

case $1 in
    major)
	bumped=$(expr $version[1] + 1)
	version[1]=$bumped
	;;
    minor)
	bumped=$(expr $version[2] + 1)
	version[2]=$bumped
	;;
    patch)
	bumped=$(expr $version[3] + 1)
	version[3]=$bumped
	;;
    *)
	echo "You need to pass one between major, minor or patch"
	;;
esac

temp=${version[*]}
new_version=${temp// /.}

sed -i 's/version = "'$old_full_version'"/version = "'$new_version'"/g' Cargo.toml
