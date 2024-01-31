# during
A utility wrapper to run a command only between certain times

## Usage

A timespec defining the allowed times a job may run is required. The format for a timespec is one or more comma separated strings like `DDD[HH:MM-HH:MM(,HH:MM-HH:MM)]`. D is a digit corresponding to the ISO 8601 day of the week (1-7, 1 = Monday, 7 = Sunday), while inside the square brackets are hour/minute ranges during which the job will be allowed to run on the given days.

By default, uses your computer's local time, but any timezone may be given.

For example, this invocation allows the command ./run.sh to run any time except between the hours of 5pm and 9pm PT on weekdays, or any time on weekends.
```
during --tz US/Pacific --timespec 12345[00:00-17:00,21:00-00:00],67[00:00-00:00] ./run.sh
```

### Notes

If a range ends with 00:00 it is taken to mean the following midnight. An end time of 23:59 would result in a job being terminated for the last minute of the day.

Ranges are additive. If the current time is within any given range, the job will be allowed to run.

## But why...?

During peak electrical rates, it can be helpful to shut down long running or power hungry jobs, such as crypto currency mining or protein folding.

Most existing solutions seemed to involve using something like systemd or cron to send a signal to your process at specific times in order to stop or start the process, but did not include any logic to prevent the service from starting outside of the allowed times.
