# SpringKeys Quote Collection

This directory contains the quote collection for the SpringKeys typing tutor, organized in JSON files by category.

## Structure

- `categories/` - Directory containing JSON files for each quote category
  - `proverbs.json` - Wisdom and proverbs
  - `tongue_twisters.json` - Tongue twisters for typing practice
  - `literature.json` - Quotes from famous literature
  - `programming.json` - Programming-related quotes
  - `humor.json` - Humorous quotes
  - `multilingual.json` - Multilingual quotes
  - `typewriters.json` - Quotes about typewriters and printing technology

## Quote Format

Each JSON file contains an array of quote objects with the following structure:

```json
{
  "text": "The quote text to type",
  "source": "The author or source of the quote",
  "difficulty": "Easy|Medium|Hard",
  "category": "CategoryName",
  "origin": "Cultural origin of the quote"
}
```

## Adding New Quotes

To add new quotes:

1. Add them to the appropriate category JSON file
2. If creating a new category, add a new JSON file and update the QuoteCategory enum in `src/quotes.rs`
3. Follow the existing format for consistency

The SpringKeys application will automatically load all quotes from these files at startup.

## Converting from the Old System

The quotes in this directory were extracted from the hardcoded collection in the original SpringKeys codebase. The JSON format provides several advantages:

1. Easier to maintain and expand the quote collection
2. No need to recompile the application to add new quotes
3. Possibility for user-contributed quotes
4. Better organization by category and difficulty

## Quote Statistics

- Total quotes: Approximately 300 across all categories
- Difficulty distribution:
  - Easy: Approximately 40% of quotes
  - Medium: Approximately 40% of quotes
  - Hard: Approximately 20% of quotes
- Language origins: English, Chinese, Japanese, Russian, French, Latin, Arabic, and many others 