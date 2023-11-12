# TODO 
Ziel : 
    in :  Datenbasis, eine Liste mit Analysemodulitäten und ggfs. Parametern. 
    out : Eine List für jedes file mit allen Ergebnissen

## Architektur

[] Führe Ports und Adapter Struktur ein
[] Teile auf in use cases
[] Verstecke Domänenlogik vor den Adaptern

## Refactor

[] Abstrahiere die Auswertung/Aggregation in generische Funktion (vlt auch via trait ?)

## Features

[] Power Analysis Durchschnitt, gewichteter Durchschnitt, Zeit in Zonen
[] 'Effective X' where X has Zones and transitations use a weighted exponential decay to consider past values
[] 'Effective time in zone' zB. wie lange war ich tatsächlich in Zone2
[] Time dependend map for zones
[] vo2 estimate
[] Fehler für alle fehlerbehafteten größen

