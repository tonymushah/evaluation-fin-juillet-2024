# Evaluation Fin Juillet 2024

Required `.env` variable

```env
DBUSER=eval_fin_juillet
DBNAME=eval_fin_juillet
DBPASS=etu001844
DBHOST=127.0.0.1:5432
DATABASE_URL=postgresql://$DBUSER:$DBPASS@$DBHOST/$DBNAME

ADMINBACKHOSTWEB=[::1]
ADMINBACKPORTWEB=50065
ADMIN_BACK_ADDR_WEB=$ADMINBACKHOSTWEB:$ADMINBACKPORTWEB
ADMIN_BACK_URL=http://$ADMINBACKHOSTWEB:$ADMINBACKPORTWEB
CLIENTBACKHOSTWEB=[::1]
CLIENTBACKPORTWEB=50069
CLIENT_BACK_ADDR_WEB=$CLIENTBACKHOSTWEB:$CLIENTBACKPORTWEB
CLIENT_BACK_URL=http://$CLIENTBACKHOSTWEB:$CLIENTBACKPORTWEB

ADMINBACKHOST=[::1]
ADMINBACKPORT=50061
ADMIN_BACK_ADDR=$ADMINBACKHOST:$ADMINBACKPORT
CLIENTBACKHOST=[::1]
CLIENTBACKPORT=50063
CLIENT_BACK_ADDR=$CLIENTBACKHOST:$CLIENTBACKPORT

ADMINHMAC=adminhmac
CLIENTHMAC=clienthmac
```
