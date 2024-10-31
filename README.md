___

REWRITE IN PROGRESS

___

# booth_archiver

A program to archive items from a user's booth wishlist

## 2023-09-28

### PC

#### Buildtime

```powershell
PS C:\Sync\Projects\booth_archiver>
measure-command { cargo clean; cargo build --release }
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 53
Milliseconds      : 406
Ticks             : 534068416
TotalDays         : 0.000618134740740741
TotalHours        : 0.0148352337777778
TotalMinutes      : 0.890114026666667
TotalSeconds      : 53.4068416
TotalMilliseconds : 53406.8416
```

#### Cached

```powershell
PS C:\Sync\Projects\booth_archiver> cargo run --release
Finished release [optimized] target(s) in 0.12s
Running `target\release\booth_archiver.exe`
getting wishlist pages => 1.830721s
extracting item numbers from pages => 2.812ms
extracting items => 14.5569ms
writing items to all_items to ron and json files => 386.5485ms
writing items to xlsx => 225.2247ms
dumping cache => 2.9µs
writing items to cache_stats to ron and json files => 529.7µs
whole program => 2.5375566s
```

#### Uncached

```powershell
PS C:\Sync\Projects\booth_archiver> cargo run --release
Finished release [optimized] target(s) in 0.31s
Running `target\release\booth_archiver.exe`
last page changed, clearing cache
getting wishlist pages => 6.2952633s
extracting item numbers from pages => 2.4816ms
extracting items => 72.475902s
writing items to all_items to ron and json files => 412.8891ms
writing items to xlsx => 229.2792ms
dumping cache => 398.6212ms
writing items to cache_stats to ron and json files => 911.8µs
whole program => 79.817074s
```
