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
