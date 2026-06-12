err=$(curl -X POST --basic --user "$1 account=$2:" https://api.alwaysdata.com/v1/site/$3/restart/ &>2);
if [[ "${err}" -eq "" ]]; then 
    echo "Redémarrage OK"
else
    echo "Erreur : ${err}"
fi