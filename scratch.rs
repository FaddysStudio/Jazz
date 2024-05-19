?# rm -rf .FaddysScratch ; mkdir .FaddysScratch

?# cat - > .FaddysScratch/header.orc

+==
sr = 96000
ksmps = 48
nchnls = 2
0dbfs = 1
-==

?# cat - > .FaddysScratch/looper.orc

+==
#include "header.orc"

#define track #STrack strget p4
STrackLeft sprintf "%s/left", STrack
STrackRight sprintf "%s/right", STrack#

instr 1

iRhythm nstrnum "rhythm"
iInstance init frac ( p1 )
iDuration init abs ( p3 )

kSwing init .5
kCycle metro2 1/iDuration, kSwing

if kCycle == 1 then

STrack strget p4
SSample strget p5

SRhythm sprintf "i %f 0 0 %s %s", iRhythm + iInstance, STrack, SSample

scoreline SRhythm, 1

endif

endin

instr rhythm

SRhythm strget p5

aRhythm [] diskin2  SRhythm

iChannels filelen aRhythm

if iChannels == 2 then

iLeft init 0
iRight init 1

else

iLeft init 0
iRight init 0

endif

$track

chnmix aRhythm [ iLeft ], STrackLeft
chnmix aSample [ iRight ], STrackRight

endin

instr track

$track

aLeft chnget StrackLeft
aRight chnget STrackRight

chnclear STrackLeft
chnclear STrackRight

aLeft clip aLeft, 1, 1
aRight clip aRight, 1, 1

iAmplitude init p5
chnmix aLeft * iAmplitude, "left"
chnmix aRight * iAmplitude, "right"

endin

instr out

aLeft chnget "left"
chnclear "left"

aRight chnget "right"
chnclear "right"

out aLeft, aRight

endin
-==
