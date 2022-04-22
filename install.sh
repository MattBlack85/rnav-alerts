OS=$(uname -s)
MACHINE=$(uname -m)
VERSION=$(curl -s https://api.github.com/repos/mattblack85/rnav-alerts/releases/latest | grep -oP '"tag_name": "\K(.*)(?=")')
PACKAGE_NAME=rnav-alerts
EXTENSION=tar.gz

case $OS in
    Linux)
	case $MACHINE in
	    x86_64)
		ARCH=linux-x64
		;;
	    armv7l)
		ARCH=armv7
		;;
	    aarch64)
		ARCH=arm64
	esac
	;;
    Darwin)
	ARCH=macos
	;;
    *)
	echo "OS not supported, please raise an issue at https://github.com/MattBlack85/rnav-alerts/issues giving the result of `uname -s`"
	;;
esac


wget https://github.com/MattBlack85/rnav-alerts/releases/download/$VERSION/$PACKAGE_NAME-$ARCH-$VERSION.$EXTENSION
tar -xvzf rnav-alerts*.tar.gz
rm rnav-alerts*tar.gz
sudo mv rnav-alerts /usr/local/bin/
