// CREATE TABLE users_profiles (
//   user_id UUID not null,
//   picture_url TEXT not null default 'https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fi0.wp.com%2Ftoppng.com%2Fuploads%2Fpreview%2Finstagram-default-profile-picture-11562973083brycehrmyv.png&f=1&nofb=1&ipt=3a19a22ba58adc9c636877ceaabc4143b3e98804cef2b9184e8991fe2cd8b87f&ipo=images',
//   url varchar(96) not null,
//   description TEXT not null default '',
//   CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES users (id),
//   PRIMARY KEY (user_id)
// )

struct PictureUrl(url::Url);
