On établi une connexion ssh. Avoir socat installé sur le RPI. Puis un daemon avec mpv en grand écran tout le temps qui est ouvert.

Enfin on lance des commande en ajoutant dans le mpvsocket des commandes: quoi lire, avancer reculer changer les sous titre etc ...

On veut donc dans le rust terminal : - le binding normal (fleche, c, v). - si on veut ouvrir un dossier on pourrait faire en sorte d'avoir une bibliothèque de ce qu'il y a de dl sur le rpi. Sinon on voudrait pouvoir stream depuis vlc sur le rpi. - Pour ca on peut imaginer un binding pour changer de mode : youtube vers fichier local (plus simple que de dl depuis le rpi) - si le client rust est lancé il peut avoir un petit serveur http qui reçois l'url de la vidéo et l'envoi directement sur le cli rust

ssh pi@raspberrypi 'DISPLAY=:0 xdotool key Left'

On va p-e faire autrement que ssh, mais exposer un port TCP :

1. Sur le rpi
   `mpv --idle --input-ipc-server=tcp://0.0.0.0:9999`

2. Pour le cli rust

```rust
use std::net::TcpStream;
use std::io::Write;

let mut stream = TcpStream::connect("192.168.1.42:9999")?;
stream.write_all(b"{ \"command\": [\"cycle\", \"pause\"] }\n")?;

```

mpv + IPC (JSON socket)

## Service permannent

1. On peut avoir mpv lancé en permanence
   `mpv --idle --input-ipc-server=/tmp/mpvsocket --fullscreen`

## Commande

1. Regarde une vidéo youtube
   `echo '{ "command": ["loadfile", "https://youtube.com/watch?v=xxxx"] }' | socat - /tmp/mpvsocket`

2. Lire une vidéo d'un fichier local
   `echo '{ "command": ["loadfile", "/home/pi/film.mp4"] }' | socat - /tmp/mpvsocket`

3. Avancer de 10 secondes
   `echo '{ "command": ["seek", 10, "relative"] }' | socat - /tmp/mpvsocket`

4. Play/Pause
   `ssh pi@raspberry "echo '{ \"command\": [\"cycle\", \"pause\"] }' | socat - /tmp/mpvsocket"`

5. Changer sous titre
   `echo '{ "command": ["cycle", "sub"] }' | socat - /tmp/mpvsocket`
