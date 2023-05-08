use crate::models::todo::TodoDto;
use sea_orm::{ConnectionTrait, DbErr, PaginatorTrait, SelectorTrait};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, PartialEq, Serialize, ToSchema)]
#[aliases(TodoDtoList=PagedList<TodoDto>)]
pub struct PagedList<T>
where
    T: Serialize,
{
    pub data: Vec<T>,
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
}

#[derive(IntoParams)]
#[into_params(parameter_in = Query)]
#[derive(Debug, PartialEq, Deserialize)]
pub struct PagedListQueryParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

impl<'db, T> PagedList<T>
where
    T: Serialize,
{
    pub async fn from_paginator<C, E>(
        entity: impl PaginatorTrait<'db, C, Selector = impl SelectorTrait<Item = E> + 'db>,
        db: &'db C,
        q: &PagedListQueryParams,
    ) -> Result<PagedList<T>, DbErr>
    where
        C: ConnectionTrait,
        E: Into<T>,
    {
        let page = q.page.unwrap_or(1);
        let page_size = q.page_size.unwrap_or(10);
        let paginator = entity.paginate(db, page_size);
        Ok(PagedList {
            total: paginator.num_pages().await?,
            page: page,
            page_size: page_size,
            data: paginator
                .fetch_page(page - 1)
                .await?
                .into_iter()
                .map(E::into)
                .collect(),
        })
    }
}

#[cfg(test)]
#[cfg(feature = "mock")]
mod tests {
    use crate::models::todo::{self, TodoDto};
    use sea_orm::{DatabaseBackend, EntityTrait, MockDatabase};

    use super::{PagedList, PagedListQueryParams};

    #[actix_web::test]
    async fn from_paginator_works() {
        let db = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![query_result()])
            .into_connection();
        let q = PagedListQueryParams {
            page: 1,
            pageSize: 2,
        };

        let queryPagedList: PagedList<TodoDto> =
            PagedList::from_paginator(todo::Todo::find(), &db, &q)
                .await
                .unwrap();
        let expected = PagedList {
            page: 1,
            pageSize: 2,
            total: query_result().len() as u64 / q.pageSize,
            data: query_result()[..q.pageSize as usize]
                .to_owned()
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<TodoDto>>(),
        };
        assert_eq!(queryPagedList, expected);
    }
    fn query_result() -> Vec<todo::Model> {
        vec![
            todo::Model {
                id: 1,
                content: Some("First thing todo".into()),
                expire_at: "0".into(),
                is_finished: true,
            },
            todo::Model {
                id: 2,
                content: Some("Na".into()),
                expire_at: "0".into(),
                is_finished: false,
            },
            todo::Model {
                id: 3,
                content: Some("foo".into()),
                expire_at: "0".into(),
                is_finished: false,
            },
            todo::Model {
                id: 4,
                content: Some("bar".into()),
                expire_at: "0".into(),
                is_finished: false,
            },
            todo::Model {
                id: 5,
                content: Some("no".into()),
                expire_at: "0".into(),
                is_finished: false,
            },
        ]
    }
}
