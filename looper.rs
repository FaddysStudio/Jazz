# Faddy's Looper

?# rm -rf .FaddysLooper ; mkdir .FaddysLooper

?# cat - > .FaddysLooper/header.orc

+==
sr = 96000
ksmps = 48
nchnls = 2
0dbfs = 1
-==

?# cat - > .FaddysLooper/looper.orc

+==
#include "header.orc"

#define track #STrack strget p4
STrackLeft sprintf "%s/left", STrack
STrackRight sprintf "%s/right", STrack#

instr 13

iRhythm nstrnum "rhythm"
iInstance init frac ( p1 )

iCycle init abs ( p3 )
kCycle metro 1/iCycle

p3 = -iCycle

kSwing jspline 1, 0, 4
kSwing += .5

if kCycle == 1 && kSwing > 0 then

STrack strget p4
SSample strget p5

SRhythm sprintf {{ i %f 0 0 "%s" "%s" }}, iRhythm + iInstance, STrack, SSample

scoreline SRhythm, 1

endif

endin

instr rhythm

SRhythm strget p5

aRhythm [] diskin2  SRhythm
p3 filelen SRhythm

iChannels lenarray aRhythm

if iChannels == 2 then

iLeft init 0
iRight init 1

else

iLeft init 0
iRight init 0

endif

out aRhythm [ iLeft ], aRhythm [ iRight ]

endin
-==
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

?# cat - > .FaddysLooper/time.orc

++= f 0 6000000

?# roll beat.rs 0/dom 1/tak 3/tak 4/dom 6/tak | tee .FaddysLooper/maqsum.sco

?# cd .FaddysLooper ; cat time.orc maqsum.sco > looper.sco

?# cd .FaddysLooper ; csound -odac looper.orc looper.sco
