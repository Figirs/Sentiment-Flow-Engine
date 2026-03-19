
pub struct SentimentAnalyzer {
    // In a real application, this would load a pre-trained model or lexicon
    // For simplicity, we'll use a basic keyword-based approach.
}

#[derive(Debug, PartialEq)]
pub enum Sentiment {
    Positive,
    Negative,
    Neutral,
}

impl SentimentAnalyzer {
    pub fn new() -> Self {
        SentimentAnalyzer {}
    }

    pub fn analyze(&self, text: &str) -> Sentiment {
        let lowercased_text = text.to_lowercase();

        let positive_keywords = vec!["great", "fantastic", "excellent", "love", "happy", "good", "amazing"];
        let negative_keywords = vec!["bad", "terrible", "horrible", "hate", "sad", "disappointed", "poor"];

        let mut positive_score = 0;
        let mut negative_score = 0;

        for keyword in positive_keywords {
            if lowercased_text.contains(keyword) {
                positive_score += 1;
            }
        }

        for keyword in negative_keywords {
            if lowercased_text.contains(keyword) {
                negative_score += 1;
            }
        }

        if positive_score > negative_score {
            Sentiment::Positive
        } else if negative_score > positive_score {
            Sentiment::Negative
        } else {
            Sentiment::Neutral
        }
    }
}

#[cfg(test)]
mod tests {
    use super:: Sentiment;
    use super::SentimentAnalyzer;

    #[test]
    fn test_positive_sentiment() {
        let analyzer = SentimentAnalyzer::new();
        assert_eq!(analyzer.analyze("This is a great product!"), Sentiment::Positive);
        assert_eq!(analyzer.analyze("I love this service, it's fantastic."), Sentiment::Positive);
    }

    #[test]
    fn test_negative_sentiment() {
        let analyzer = SentimentAnalyzer::new();
        assert_eq!(analyzer.analyze("This is a terrible product."), Sentiment::Negative);
        assert_eq!(analyzer.analyze("I hate this service, it's horrible."), Sentiment::Negative);
    }

    #[test]
    fn test_neutral_sentiment() {
        let analyzer = SentimentAnalyzer::new();
        assert_eq!(analyzer.analyze("The weather is okay."), Sentiment::Neutral);
        assert_eq!(analyzer.analyze("This is a sentence."), Sentiment::Neutral);
        assert_eq!(analyzer.analyze("Neither good nor bad."), Sentiment::Neutral);
    }

    #[test]
    fn test_mixed_sentiment() {
        let analyzer = SentimentAnalyzer::new();
        // Should lean towards positive due to more positive words
        assert_eq!(analyzer.analyze("It's good but has some bad parts."), Sentiment::Neutral);
        assert_eq!(analyzer.analyze("Great product, but the support was bad."), Sentiment::Neutral);
    }
}
