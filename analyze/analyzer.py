import re
from datetime import datetime

import pandas as pd

__all__ = [
    "parse_access_log", "parse_vmstat_log", "vmstat_fields"
]


def parse_access_log(file, uri_path_map: dict):
    with open(file) as fp:
        logs = [
            dict([d.split(":", 1) for d in line.split("\t") if len(d) > 2])  # noqa
            for line in fp.read().strip().split("\n")
        ]

    mapper = {h: re.compile(p) for h, p in uri_path_map.items()}

    def map_func_to_key(method, uri):
        for _h, _p in mapper.items():
            m = _p.match(f"{method.upper()} {uri}")
            if m is not None:
                return _h, list(m.groups())
        return None, []

    for log in logs:
        log["time"] = datetime.strptime(log["time"], "%d/%b/%Y:%H:%M:%S %z")
        log["handler"], log["params"] = map_func_to_key(log["method"], log["uri"])

    return pd.DataFrame(logs, dtype="f8")


def parse_vmstat_log(file, readable_column=False):
    with open(file) as fp:
        logs = [line.split() for line in fp.read().strip().split("\n")]

    data = [list(map(int, log[:-2])) for log in logs[2:]]

    df = pd.DataFrame(data, columns=logs[1][:-1])
    df["datetime"] = [datetime.strptime(f"{log[-2]} {log[-1]}", "%Y-%m-%d %H:%M:%S") for log in logs[2:]]
    if readable_column:
        df = df.rename(columns=vmstat_fields)
    return df


vmstat_fields = {
    "r": "[Procs] The number of runnable processes (running or waiting for run time).",
    "b": "[Procs] The number of processes in uninterruptible sleep.",

    "swpd": "[Memory] The amount of virtual memory used.",
    "free": "[Memory] The amount of idle memory.",
    "buff": "[Memory] The amount of memory used as buffers.",
    "cache": "[Memory] The amount of memory used as cache.",
    "inact": "[Memory] The amount of inactive memory. ",
    "active": "[Memory] The amount of active memory. ",

    "si": "[Swap] Amount of memory swapped in from disk (/s).",
    "so": "[Swap] Amount of memory swapped to disk (/s).",

    "bi": "[IO] Blocks received from a block device (blocks/s).",
    "bo": "[IO] Blocks sent to a block device (blocks/s).",

    "in": "[System] The number of interrupts per second, including the clock.",
    "cs": "[System] The number of context switches per second.",

    "us": "[CPU] Time spent running non-kernel code.  (user time, including nice time)",
    "sy": "[CPU] Time spent running kernel code.  (system time)",
    "id": "[CPU] Time spent idle.  Prior to Linux 2.5.41, this includes IO-wait time.",
    "wa": "[CPU] Time spent waiting for IO.  Prior to Linux 2.5.41, included in idle.",
    "st": "[CPU] Time stolen from a virtual machine.  Prior to Linux 2.6.11, unknown.",

    "datetime": "Datetime"
}

if __name__ == "__main__":
    _df = parse_vmstat_log("sample/vmstat.log", True)
    print(_df.dtypes)

    uri_path = {
        "routePostAd": r"POST /slots/(.+?)/ads$",
        "routeGetAd": r"GET /slots/(.+?)/ad$",
        "routeGetAdWithId": r"GET /slots/(.+?)/ads/(\d+?)$",
        "routeGetAdAsset": r"GET /slots/(.+?)/ads/(\d+?)/asset$",
        "routeGetAdCount": r"POST /slots/(.+?)/ads/(\d+?)/count$",
        "routeGetAdRedirect": r"GET /slots/(.+?)/ads/(\d+?)/redirect$",
        "routeGetReport": r"GET /me/report$",
        "routeGetFinalReport": r"GET /me/final_report$",
    }

    _df = parse_access_log("sample/access.log", uri_path)
    print(_df.dtypes)
