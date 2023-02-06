# Keypa

## Projektbeschreibung

Schlüsselverwaltung für Universitäten

### Projektziele

Eine Schlüsselverwaltung für Universitäten ist ein System, das für den sicheren und effizienten Umgang mit den verschiedenen Schlüsseln auf dem Campus verantwortlich ist. Ziel ist es, den Zugang zu Gebäuden, Räumen und Einrichtungen zu kontrollieren und zu verwalten, um sicherzustellen, dass nur berechtigte Personen Zugang haben. Darüber hinaus trägt sie zur Meldung bei Verlusten und automatischer Sperrung bei ablaufenden Berechtigung.


### Funktionen und Aufbau
#### Aufbau
* Ordner frontend enthaelt das gesamte Frontend
* Ordner api enthaelt die REST API und die Logik der Anwendung
  * Aufteilung des Api Ordner in 2 Ordner Strukturen
  1. api Definition der Api als Swagger Dokumentation und Verbreitung als REST Schnittstelle
  2. crud Verbindung mit der Datenbank und Verarbeiten, Speichern der angefragten Daten
Ordner mock enthaelt einen einfachen Datengenerator

#### Funktionen
* Erstellen und Bearbeiten von Antraegen
* Lueckenlose Auskunft ueber Antrag- und Kartennutzungsverlauf
* Ausgabe von Keycards
* Verwalten von Keycards
* Rollensystem
* Archivierung abgelaufener Antraege und Keycards
* Responsive Design

## Team

### Vorstellung

* nikoklaiber Niko Klaiber nilotrim@gmail.com 103121
* stefhol Stefan Höfler hoefle15@uni-passau.de 81703
* kobeeeeeey Korbinian Anzenberger kobeeeeeey@gmail.com
* noahbundschuh Noah Bundschuh noah.bundschuh@gmail.com 77421
* maxidoerfler Maximilian Dörfler maxidoerfler9@gmail.com 89620


### Zuständigkeiten

* Niko Klaiber: Projektleitung, Pflichtenheft, Projektmanagment, Kommunikation Kunde, Testen
* Stefan Höfler: Entwicklung, Pflichtenheft
* Korbinian Anzenberger: Pflichtenheft, Testen, Presaentationen
* Noah Bundschuh: Pflichtenheft, Testen, Presaentationen
* Maximilian Dörfler: Pflichtenheft, Testen, Presaentationen

## Guidelines zur Nutzung dieses Repositorys

### Allgemeine Hinweise und Vorgaben

* Das Repository besteht im initialen Stand aus einem einzelnen Main-Branch. Versuchen Sie bei der Arbeit am Projekt darauf zu achten, dass sich in diesem Branch stets die aktuelle lauffähige und fehlerfreie Version Ihrer Anwendung befindet. Nutzten Sie für die eigentliche Entwicklung ggf. weitere Branches.
* Gehen Sie sorgfältig bei der Erstellung von Issues und *Commit Messages* vor: Die Qualität dieser Artefakte fließt nicht in die Bewertung ein, trotzdem sollten Sie versuchen, Ihr Vorgehen anhand nachvollziehbarer Versionseinträge und klarere Aufgabenbeschreibung gut zu dokumentieren.
* Halten Sie diese und ggf. weitere Readme-Datei(en) stets aktuell.
* Diese sollte auch wichtige Informationen für die Nutzung und die initiale Inbetriebnahme beinhalten (Handbuch).
* Achten Sie insbesondere darauf anzugeben, welche externen Abhängikeiten oder Frameworks ihrem Projekt zugrunde liegen sowie auf die Lesbarkeit ihres finalen Codes.
