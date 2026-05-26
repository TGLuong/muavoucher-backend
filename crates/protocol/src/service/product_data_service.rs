use std::collections::VecDeque;

use tokio::sync::mpsc::{self, Sender};
use tokio_stream::{Stream, StreamExt, wrappers::ReceiverStream};
use tonic::{Request, Response, Status, Streaming};
use uuid::Uuid;

use crate::proto::protocol::{ProductDataResponse, ProductRequest, product_data_service_server::ProductDataService};

pub struct ProductDataServiceImpl {
    out: VecDeque<(Uuid,)>,
}

#[tonic::async_trait]
impl ProductDataService for ProductDataServiceImpl {
    type ProcessDataStream = ReceiverStream<Result<ProductRequest, Status>>;
    async fn process_data(&self, request: Request<Streaming<ProductDataResponse>>) -> Result<Response<Self::ProcessDataStream>, Status> {
        let in_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(100);
        let runner = ProductDataServiceRunner::new(in_stream, tx);
        let receiver = ReceiverStream::new(rx);
        Ok(Response::new(receiver))
    }
}

pub struct ProductDataServiceRunner {
    pub id: Uuid,
    pub in_stream: Streaming<ProductDataResponse>,
    pub out_stream: Sender<Result<ProductRequest, Status>>,
}

impl ProductDataServiceRunner {
    pub fn new(in_stream: Streaming<ProductDataResponse>, out_stream: Sender<Result<ProductRequest, Status>>) -> Self {
        let id = Uuid::now_v7();
        Self { id, in_stream, out_stream }
    }
}

impl Stream for ProductDataServiceRunner {
    type Item = ();

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        todo!()
    }
}
