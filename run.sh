set -eu

cargo deb -p package
sudo apt reinstall ./target/debian/time-tracker_0.1.0_amd64.deb
echo
echo
echo
time-tracker
