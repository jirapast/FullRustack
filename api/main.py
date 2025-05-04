from fastapi import FastAPI

app = FastAPI()

@app.get("/api")
async def read_root():
    return {
        "key": "val",
        "id": 0,
        "name": "my_name",
    }

# uvicorn main:app --reload
