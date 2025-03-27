# GTFS Parsing
This is a project that seeks to parse [GTFS](https://gtfs.org/documentation/overview/) schedule data into a well-typed format. It will first and foremost be used for MTA subway data since that's what interests me the most, but I do think it would be interesting to provide basic support for other systems (that's the point of a standard after all). 

## Testing
The non-archived testing data has files too large for standard Git, so instead I store the zip file that the tests were built around. I would like to eventually automate unzipping the test files but until then, you need to manually extract each file into the `test_data` directory. E.g. `test_data/agency.txt`, etc. You also must create an abbreviated version of `stop_times.txt`. I used the following command to achieve this: `head -n 10001 test_data/stop_times.txt > test_data/stop_times_abbrev.txt`.
