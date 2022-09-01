#[cfg(test)]
mod twetch_pay_tests {
    use anyhow::Result;
    use twetch_sdk::ABIv1Schema;

    #[test]
    fn schema() -> Result<()> {
        let schema = match ABIv1Schema::new() {
            Ok(v) => v,
            Err(e) => {
                return anyhow::bail!(format!("{:?}", e));
            }
        };

        let postSchema = schema.actions.get("twetch/post@0.0.1").unwrap();

        assert_eq!(postSchema.args.len(), 31);

        let likeSchema = schema.actions.get("twetch/like@0.0.1").unwrap();

        assert_eq!(likeSchema.args.len(), 9);

        Ok(())
    }
}
