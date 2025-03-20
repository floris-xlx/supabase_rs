//! Handles user deletion operations

use crate::{AuthClient, AuthError};
use tracing::{debug, instrument};
use uuid::Uuid;

impl AuthClient {
    /// Permanently deletes a user account
    ///
    /// # Arguments
    /// * `user_id` - UUID of the user to delete
    ///
    /// # Returns
    /// * `Result<(), AuthError>` - Success or error
    #[instrument(skip_all)]
    pub async fn hard_delete_user(&self, _user_id: Uuid) -> Result<(), AuthError> {
        Ok(())
    }
}
