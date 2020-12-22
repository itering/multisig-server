CREATE TABLE wallets (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  name VARCHAR(128) NOT NULL,
  address CHAR(66) NOT NULL,
  parties TEXT NOT NULL,
  threshold int(3) unsigned NOT NULL,
  balance int(11) NOT NULL,
  PRIMARY KEY (`id`)
)
