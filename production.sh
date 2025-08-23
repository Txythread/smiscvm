# Generate the instruction table using smiscasm
mkdir -p instruction-gen
cd instruction-gen
smiscasm --generate-instruction-table
cd ..


# Make a production build and store it in the /usr/local/bin
cargo build --release 1>/dev/null 2>/dev/null

if [ $? -ne 0 ]; then
	echo "Build failed!" 1>&2
	exit 1
fi


cargo test 1>/dev/null 2>/dev/null

if [ $? -ne 0 ]; then
	echo "Tests failed!" 1>&2
	exit 2
fi

sudo mv target/release/smiscvm /usr/local/bin
