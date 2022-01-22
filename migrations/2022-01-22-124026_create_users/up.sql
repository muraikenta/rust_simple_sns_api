CREATE TABLE users(
	id int NOT NULL AUTO_INCREMENT, 
  email varchar(255) NOT NULL, 
  password varchar(255) NOT NULL, 
  name varchar(255) NOT NULL, 
  icon_url varchar(255) NULL, 
  created_at datetime (6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6), 
  updated_at datetime (6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6), 
  UNIQUE INDEX IDX_e12875dfb3b1d92d7d7c5377e2 (email), 
  PRIMARY KEY (id)
) ENGINE = InnoDB