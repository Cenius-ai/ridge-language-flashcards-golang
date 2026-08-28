# Ridge Usage Guide

## Access the Application

Once the development server is running (`cargo run`), open a browser and go to:
```
http://localhost:8080
```

All pages share a common layout (`templates/layout.html`) and automatically adapt to your system’s light/dark mode preference.

---

## Dashboard

**URL:** `/`

Displays your study progress:
- Number of cards due for review
- Total cards in the collection

A button or link lets you start a study session.

## Study Session

**URL:** `/study`

Presents one flashcard at a time. After revealing the answer, you rate how well you recalled it. The app uses the SM‑2 algorithm to schedule the next review.

**Answer Submission**  
`POST /study/answer`  
This endpoint receives your rating and updates the card’s schedule.

When all due cards have been reviewed, you are redirected to the study completion page.

## Study Complete

**URL:** (rendered after finishing a study session)  
Confirms that you have completed the current batch of cards.

## Card Management

### List All Cards
**URL:** `/cards`  
Shows a table of all flashcards with options to edit or delete each one.

### Create a New Card
**URL:** `/cards/new`  
Form to enter the front and back content of a new card.

**Submission:** `POST /cards` – creates the card.

### Edit a Card
**URL:** `/cards/{id}/edit`  
Displays a form pre‑filled with the card’s current data.

**Update:** `POST /cards/{id}` – saves changes.

### Delete a Card
**URL:** `POST /cards/{id}/delete`  
Removes the card from the collection.

---

## Navigation

All pages include navigation links at the top:
- **Dashboard** (`/`)
- **Study** (`/study`)
- **Cards** (`/cards`)

Use these to move between sections without manually typing URLs.