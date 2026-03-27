use std::thread;

use bambangshop::{compose_error_response, Result};
use rocket::http::Status;
use crate::model::notification::Notification;
use crate::model::product::Product;
use crate::model::subscriber::Subscriber;
use crate::repository::subscriber::SubscriberRepository;

pub struct NotificationService;

impl NotificationService {
}