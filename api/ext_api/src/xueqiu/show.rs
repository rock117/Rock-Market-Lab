use std::collections::HashMap;
use common::http;
use anyhow::Result;
use tracing::info;

pub async fn show(user_id: &str) -> Result<String> {
    let url = format!("https://xueqiu.com/statuses/original/show.json?user_id={user_id}");
    println!("url = {}", url);
    let mut headers = HashMap::new();
    headers.insert("Cookie", "cookiesu=881766116958792; device_id=6abad25ea8f98b32353f6372bcf26ef5; smidV2=202512191202516a6915fda6c65750527498808624e389003d16a486006bb60; s=c312iqw708; remember=1; xq_a_token=aaefccfc727b3ef834cac59644f8e537204a4f01; xqat=aaefccfc727b3ef834cac59644f8e537204a4f01; xq_id_token=eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJ1aWQiOjUyMzQ3NzI3MjAsImlzcyI6InVjIiwiZXhwIjoxNzcwNTYwNTcyLCJjdG0iOjE3Njc5Njg1NzIyNDYsImNpZCI6ImQ5ZDBuNEFadXAifQ.XngxXgAN7rdlJViCNOEWoiM7JsZiTGvuRXjYuunuqJ281SANRuJj7ZU1kjhtheem6UZbuPx3IDwHwjroAA810ATdgupnde99YosoP5cvvUaj5M5B9DZTp4o7U_zCZDo_WID4D5YXZKGuRD08LZKLfn-l6KXTsqHlGbG0NyrenT6V080qRkXm2vaS83qgi0otEGK6rGJP7LVvnfuVslMtUCVmbbHQOP3F_lctMbfVGWZbai2sbB2RjAN_A-_ejBH_XnUKr41E49XzgpALPGUP-sUMar6A7XayWLpkVobDP_BbJFEEs_-Su3LoFL82_AvA_r6d2BA_pJdp0sHATv0zuQ; xq_r_token=1d6859342f30386d92d215826a5b7be4830a60b7; xq_is_login=1; u=5234772720; bid=40a2284578a8d213fb521fa7018869b6_mk7uh6gr; .thumbcache_f24b8bbe5a5934237bbc0eda20c1b6e7=MpWOiBpZFaDNO91ba1c6hP6ak9FVPh9HbMTlRCEZX7AJ0XQ1vBLmXwxV0SxMf8czTBzG6q/tp3Y6KIogO/r0kQ%3D%3D; acw_tc=ac11000117680497600534249e46684837cea5a77e543a01a558350f508dd2; ssxmod_itna=1-QqAO0IqUrxCii7G7DgDeLDKMYhDuYDl4BtGRjDIqGQGcD8xiKDHKmpDfr=WbgAxDK4W/c0y=PhG=DEeDlhdeDZDGIdDqx0EiUYDCy_sy0gW4QPWmml9hsNoiA=jkyGZnlGI=8FH/rM=BktP/L64T3eDU4GnD06xGOvk4xGG04GwDGoD34DiDDPjD03Db4D_K1bD7hTr=Patxi3DA4DjBCb7/rHtk0bDe_TQhDDBDGtDzk7t6eDADAfNVxDlnaHgl1bDbhubaeDg3ow_5AWomGTDjqPD/SIcVhuhOyZsF6WR=NMIjlBwxBQD0FoBnA7WKm2v8cEnFbY0ze2Nbbq42GKGgY0=8iqmiH9xKuYPjYPAiYAGq/2b97Y_7eTTeO93wOeoGetiYByVeYx=AQ9xmGhzYGO9TTWikmnbGxkFTqtTYGutSxiU2NtO=Tu=KgxYG4mqm_APPD; ssxmod_itna2=1-QqAO0IqUrxCii7G7DgDeLDKMYhDuYDl4BtGRjDIqGQGcD8xiKDHKmpDfr=WbgAxDK4W/c0y=PhG=DoDA4ZdnuAbh4pDWz//1MvwsbBialNxD");
    let resp = http::get(&url, Some(&headers)).await?;
    let text = http::to_string(resp).await?;
    Ok(text)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_show() {
        let t = show("2202689376").await.unwrap();
        println!("{t}");
    }
}