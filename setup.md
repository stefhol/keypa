# Einrichtung Keypa

## Einrichtung Docker
Hier wird auf die 
[Offizielle Installation Anleitung verwiesen](https://docs.docker.com/engine/install/ubuntu/)

Folgendes bezieht sich auf Ubuntu
1. Set up the repository
```
sudo apt-get update

sudo apt-get install \
    ca-certificates \
    curl \
    gnupg \
    lsb-release
```
2. Add Docker’s official GPG key:
```
sudo mkdir -p /etc/apt/keyrings

 curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
```
3. Use the following command to set up the repository:
```
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
```
###  Install Docker Engine
```
sudo apt-get update
```
### Install Docker Engine, containerd, and Docker Compose.
```
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-compose-plugin
```

### Post Installation Steps
To run Docker without root privileges, see Run the Docker daemon as a non-root user (Rootless mode).
```
sudo groupadd docker
sudo usermod -aG docker $USER
```
Add to Autostart
```
 sudo systemctl enable docker.service

 sudo systemctl enable containerd.service
```
**Restart System**
```
sudo reboot
```

## Einstellung Github
Vorraussetzung
* Zugriff zum Github Repository
* [Generieren sie einen Github Token](https://github.com/settings/tokens/new) mit read:packages Berechtigung
* Speichern des Tokens zum Beispiel ghp_fUEmAQS8wHBeDn1P........
## Installation Keypa
### Login in die Github Container Registry
Keypa ist bereits als Dockerfile vorhanden, dieses befindet sich im Github Repository, um dies zu benutzen muss allerdings ein Login erfolgen. 

Dazu benoetigen wir unseren Github Nutzernmaen und den vorher generieten Token mit read:packages Berechtigung

```
sudo docker login ghcr.io -u GITHUB_USERNAME -p GITHUB_TOKEN
```
Fuer neue Versionen muss dieser Login Vorgang wiederholt werden.
###
## Finale Installation auf Server und Einrichtung
Fuer die Installation kann ein docker-compose.yml Datei verwendet werden, ein Beispiel hierfuer ist in der Github Repository

In dieser Beispiel Konfiguration wird die Datenbank mitgeliefert, diese ist nur lokal erreichbar.
Der Keypa Server laeuft unter Port 80, sollte dieser belegt sein, kann das Ports Attribute veraendert werden.

Als Beispiel 
```
ports:
    - "81:80"
```
Laesst Keypa nur auf Port 81 horchen.
### Externe Datenbank
Um eine Externe Datenbank zu verwenden muss der DATABASE_URL angepasst werden, Keypa funktioniert nur mit PostgreSql.
```
postgres://db_nutzer:db_passwort@ip_server:port_server/tabelle
```
### SMTP Server
Der SMTP Server muss extern Verwaltet werden, dieser kann ueber die Bereitgestellten Enviroment Variablen (siehe docker-compose.yml) veraendert werden.

## Einstellung der Erinnerungsemails Intervals

Das Erinnerungsemailsinterval kann anhand der Enviorement Variable EMAIL_SHEDULE eingestellt werden.

```
  sec  min   hour   day of month   month   day of week   year
  0   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2
```
Das Format des Intervals entspricht die von dem Linux Tool crontab.
Hier ein paar Beispiele:
1. Jede Stunde bei Minute 0 und Sekunde 0
```
0 0 * * * * *
```
2. Jeden Tag bei Stunde 0 Minute 0 und Sekunde 0
```
0 0 0 * * * *
```
3. Jeden Montag um 00:00:00
```
0 0 0 * * Mon *
```



## Starten des Servers

Da nun die docker-compose.yml richtig eingestellt wurde, koennen wir den Server starten.
Die docker-compose.yml muss in das gewuenschten Installationverzeichnis. Wenn wir nun den Command 
```
sudo docker-compose up -d
```
 benutzen wird die Datenbank im gleichen Ordner unter ./postgres gespeichert (siehe Volume im Compose File).
Der Keypa Server fuehrt die noetigen Migrationen durch und der Service ist betriebsbereit.

### Generierung Demo Daten
Finden der laufenden Keypa Container Id
```
sudo docker ps
```
---Output-----
```
CONTAINER ID   IMAGE                                            COMMAND                  CREATED          STATUS          PORTS                                       NAMES
2da1799bea58   ghcr.io/wirtschaftsinformatik-passau/keypa:main   "/docker-entrypoint.…"   53 minutes ago   Up 53 minutes   0.0.0.0:80->80/tcp, :::80->80/tcp           keypa-keypa-1
ab4104794777   postgres:11-alpine                               "docker-entrypoint.s…"   53 minutes ago   Up 53 minutes   0.0.0.0:5432->5432/tcp, :::5432->5432/tcp   keypa-keypa_db-1
```
Der erste Eintrag hat das Keypa Container Image mit dieser CONTAINER ID koennen wir nun in den Container
```
sudo docker exec -it 2da1799bea58 bash 
```
Nun reicht es den Command `/usr/local/app/mock` auszufuehren und es wird eine Demo Raum Datenbank und Demo Nutzer Datenbank erstellt.

### Aktualisierung von Keypa
Sollte eine neue Version zur Verfuegung stehen, wird das System auf Github ueber Github Actions aktualisiert und als Docker Image bereitgestellt. Dies geschieht automatisch falls ein neuer Commit in den Main Branch gepushed wird.

 Ob dies erfolgt ist kann [hier](https://github.com/Wirtschaftsinformatik-Passau/softwareprojekt-gruppe-1/actions/workflows/build_complete.yaml) eingesehen werden.

Um nun Keypa zu  aktualisieren navigieren Sie in den Ordner in dem auch diese docker-compose.yml enthalten ist.
Und fuehren sie die unten stehenden Commands aus.
```
sudo docker login ghcr.io -u GITHUB_USERNAME -p GITHUB_TOKEN # Login
sudo docker-compose down # Keypa herunterfahren
sudo docker-compose pull # Keypa aktualiesieren
sudo docker-compose up -d # Keypa hochfahren
```

