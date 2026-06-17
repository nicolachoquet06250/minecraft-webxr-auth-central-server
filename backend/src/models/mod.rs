pub mod user;
pub mod server;
pub mod avatar;
pub mod server_visit;
pub mod server_favorite;
pub mod friend_request;
pub mod friendship;

pub use user::Entity as User;
pub use server::Entity as Server;
pub use avatar::Entity as Avatar;
pub use server_visit::Entity as ServerVisit;
pub use server_favorite::Entity as ServerFavorite;
pub use friend_request::Entity as FriendRequest;
pub use friendship::Entity as Friendship;
