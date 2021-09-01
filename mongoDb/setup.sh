mongo <<EOF
use admin;
db.auth('root', 'dot_marketplace');
use authentication-server;
db.createUser({user: 'marketplace_admin', pwd: 'marketplace_pwd', roles: [{role: 'readWrite', db: 'authentication-server'}]});
EOF
