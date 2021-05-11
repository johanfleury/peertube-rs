/**
 * <https://docs.joinpeertube.org/api-rest-reference.html#tag/Accounts>
 */
pub struct Accounts {
    config: crate::Config,
}

impl Accounts {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * Get an account.
     */
    pub async fn get(&self, name: &str) -> crate::Result<crate::data::Account> {
        crate::Api::get(&self.config, format!("/accounts/{}", name).into()).await
    }

    /**
     * List videos of an account.
     */
    pub async fn videos(&self, name: &str, params: &crate::param::Videos) -> crate::Result<crate::Pager<crate::data::Video>> {
        let request = crate::Request {
            path: format!("/accounts/{}/videos", name),
            params,
            auth: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * List accounts.
     */
    pub async fn all(&self, pagination: &crate::param::Pagination) -> crate::Result<crate::Pager<crate::data::Account>> {
        let request = crate::Request {
            path: "/accounts".to_string(),
            params: pagination,
            auth: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * List video channels of an account.
     */
    pub async fn video_channels(&self, name: &str, params: &crate::param::Channels) -> crate::Result<crate::Pager<crate::data::Channel>> {
        let request = crate::Request {
            path: format!("/accounts/{}/video-channels", name),
            params,
            auth: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * List ratings of an account.
     */
    pub async fn ratings(&self, auth: &crate::data::Token, name: &str, params: &crate::param::Ratings) -> crate::Result<crate::Pager<crate::data::Channel>> {
        let request = crate::Request {
            path: format!("/accounts/{}/ratings", name),
            params,
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn account() {
        let (api, _) = crate::test::api();
        let account = tokio_test::block_on(
            api.accounts.get(&crate::test::username())
        ).unwrap();
        assert_eq!(account.display_name, crate::test::username());
    }

    #[test]
    fn account_videos() {
        let (api, _) = crate::test::api();
        let videos = tokio_test::block_on(
            api.accounts.videos(&crate::test::username(), &crate::param::Videos::default())
        );
        assert!(videos.is_ok());
    }

    #[test]
    fn accounts() {
        let (api, _) = crate::test::api();
        let accounts = tokio_test::block_on(
            api.accounts.all(&crate::param::Pagination {
                count: Some(2),
                .. Default::default()
            })
        ).unwrap();
        assert_eq!(accounts.data.len(), 2);
    }

    #[test]
    fn account_video_channels() {
        let (api, _) = crate::test::api();
        let channels = tokio_test::block_on(
            api.accounts.video_channels(&crate::test::username(), &crate::param::Channels::default())
        );
        assert!(channels.is_ok());
    }

    #[test]
    fn accountratings() {
        let (api, token) = crate::test::api();
        let ratings = tokio_test::block_on(
            api.accounts.ratings(&token, &crate::test::username(), &crate::param::Ratings::default())
        );
        assert!(ratings.is_ok());
    }
}
