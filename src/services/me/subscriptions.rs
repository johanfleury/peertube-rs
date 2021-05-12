pub struct Subscriptions {
    config: crate::Config,
}

impl Subscriptions {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * Get my user subscriptions.
     */
    pub async fn all(&self, auth: &crate::data::Token, pagination: &crate::param::Pagination) -> crate::Result<crate::Pager<crate::data::Subscription>> {
        let request = crate::Request {
            path: "/users/me/subscriptions".into(),
            params: crate::Params::Query(pagination),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Add subscription to my user.
     */
    pub async fn add(&self, auth: &crate::data::Token, uri: &str) -> crate::Result<()> {
        let params = crate::param::Subscription {
            uri: uri.to_string(),
        };

        let request = crate::Request {
            path: "/users/me/subscriptions".into(),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::post::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * Get if subscriptions exist for my user.
     */
    pub async fn exist<T: ToString>(&self, auth: &crate::data::Token, uris: &[&T]) -> crate::Result<std::collections::HashMap<String, bool>> {
        let params: Vec<_> = uris.iter().map(|x| ("uris", x.to_string())).collect();

        let request = crate::Request {
            path: "/users/me/subscriptions/exist".into(),
            params: crate::Params::Query(params),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * List videos of subscriptions of my user.
     */
    pub async fn videos(&self, auth: &crate::data::Token, params: &crate::param::Videos) -> crate::Result<crate::Pager<crate::data::Video>> {
        let request = crate::Request {
            path: "/users/me/subscriptions/videos".into(),
            params: crate::Params::Query(params),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Get subscription of my user.
     */
    pub async fn get(&self, auth: &crate::data::Token, handle: &str) -> crate::Result<crate::data::Subscription> {
        let request = crate::Request {
            path: format!("/users/me/subscriptions/{}", handle),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Delete subscription of my user.
     */
    pub async fn delete(&self, auth: &crate::data::Token, handle: &str) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/users/me/subscriptions/{}", handle),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::delete::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn all() {
        let (api, token) = crate::test::api();

        let subscriptions = tokio_test::block_on(
            api.me.subscriptions.all(&token, &crate::param::Pagination::default())
        );
        assert!(subscriptions.is_ok());
    }

    #[test]
    fn add() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.me.subscriptions.add(&token, "lqdn@video.lqdn.fr")
        );
        assert!(status.is_ok());
    }

    #[test]
    fn exist() {
        let (api, token) = crate::test::api();

        let results = tokio_test::block_on(
            api.me.subscriptions.exist(&token, &[&"lqdn@video.lqdn.fr"])
        );
        assert_eq!(results.unwrap().get("lqdn@video.lqdn.fr"), Some(&false));
    }

    #[test]
    fn videos() {
        let (api, token) = crate::test::api();

        let videos = tokio_test::block_on(
            api.me.subscriptions.videos(&token, &crate::param::Videos::default())
        );
        assert!(videos.is_ok());
    }

    #[test]
    fn get() {
        let (api, token) = crate::test::api();

        let subscription = tokio_test::block_on(
            api.me.subscriptions.get(&token, "lqdn@video.lqdn.fr")
        );
        assert!(subscription.is_ok());
    }

    #[test]
    fn delete() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.me.subscriptions.delete(&token, "lqdn@video.lqdn.fr")
        );
        assert!(status.is_ok());
    }
}
