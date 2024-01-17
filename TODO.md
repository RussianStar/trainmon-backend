# TODO 
Ziel : 
    in :  Datenbasis, eine Liste mit Analysemodulitäten und ggfs. Parametern. 
    out : Eine List für jedes file mit allen Ergebnissen

## Architektur


## Refactor


## Features

[] Power Analysis , gewichteter Durchschnitt
[] 'Effective X' where X has Zones and transitations use a weighted exponential decay to consider past values
[] 'Effective time in zone' zB. wie lange war ich tatsächlich in Zone2
[] Time dependend map for zones
[] vo2 estimate
[] Fehler für alle fehlerbehafteten größen
[] endpoints für daten einlesen : oura/metriken
    - speziell für alle TrainingPeaks metriken :
[] endpoint für neue Daten sind da
[] python service der neue Daten via tp lädt

## Visuals

[] htmx basis
[] button für update via python
[] zweistuffiges interaktives update : erst python dann import
[] htmx plots via images, nice !
[] plot effizienzmaß vo2 pro TSS


### Gedanken

Wie soll ich die Metriken darstellen ? 
- Wie TrainingPeaks, also Time, Name, Value
Vorteile : Einfach zum importieren und ohne eine große Einschränkung. Man macht wenig und daher verbaut man ishc auch wenig.
Nachteile: Mehr Aufwand bei allen weiteren Funktionen.

- 'Aggregiert' für jeden Tag also für jeden Tag : (Tag, Weight, Hr, HRV, Schlaf und so weiter)
Vorteile: Alles schon aggregiert, der Tag ist die natürliche Einheit.
Nachteile: Mehr Aufwand beim input

Conclusio des 11.1 im Jahre des Herrn 2024 :
"Lieber mehr beim Import machen, ja dann hab ich da meine eigenen Probleme (zb. mögen mache Metriken fehlen, neue Metrik = Codechange), aber dafür ist dann alles schnell. 

Und komm was wird man den revolutionäres finden ?
Philosopie ist sowieso, lieber langweilig auf das bekannte setzen und das visualisieren. Big Picture vor small picture. Sollte man dann einen neuen tollen Gesundheits/Regenerationswert finden, entweder komplett neu oder nur nicht implementiert aber bekannt, dann muss der eigebaut werden.
"



### Aktuell 

[] Füge navigation hinzu mit login und globalen username
