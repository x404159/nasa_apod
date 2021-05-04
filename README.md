# nasa_apod

This command line app is build using **APOD**(Astronomy Picture of the Day) api from NASA.
i have build this app to practice rust programming language .

## How to run

you will need to install rust and git on your system to run this cli app.

open the terminal and enter:

	git clone https://github.com/code-here/nasa_apod
	cd nasa_apod

export/set this environment variable:

- API_KEY

in fish shell you can set using below command:

	set -gx API_KEY "your api key"


you can get the api key from [here](https://api.nasa.gov).

then run:

	cargo build --release

the executable can be found at target/release/nasa_apod

## Using the executable

to get today's article and picture:

	./nasa_apod

for a range of date:

	./nasa_apod --start_date <date> --end_date <date>

for a specific date:

	./nasa_apod --date <date>
