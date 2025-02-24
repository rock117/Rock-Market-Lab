1. get all
let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
let stock_daily_basics: Vec<stock_daily_basic::Model> = stock_daily_basic::Entity::find()
    .filter(stock_daily_basic::Column::TsCode.eq(ts_code))
    .order_by_desc(stock_daily_basic::Column::TradeDate)
    .all(conn)
    .await?;
2. paging
let dates: Vec<trade_calendar::Model> = trade_calendar::Entity::find()
   .filter(trade_calendar::Column::CalDate.gte(&year_begin))
   .filter(trade_calendar::Column::IsOpen.eq(1))
   .order_by_asc(trade_calendar::Column::CalDate)
   .paginate(conn, 1)
   .fetch_page(0)
   .await?;
3. get by id
let entity = stock::Entity::find_by_id(&ts_code).one(&self.0).await;