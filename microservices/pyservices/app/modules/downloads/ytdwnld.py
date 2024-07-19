import asyncio
from concurrent.futures import ThreadPoolExecutor
from fastapi import APIRouter, HTTPException, BackgroundTasks
from pydantic import BaseModel
from yt_dlp import YoutubeDL
import os
import aiohttp
import json
import logging

# Настройка логгирования
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

router = APIRouter()


class VideoRequest(BaseModel):
    url: str


class VideoResponse(BaseModel):
    filename: str


class VideoInfoResponse(BaseModel):
    title: str
    duration: float
    view_count: int


class DownloadStatusResponse(BaseModel):
    status: str
    progress: float = None


download_progress = {}


@router.post("/ytdwnld")
async def download_video(request: VideoRequest, background_tasks: BackgroundTasks):
    url = request.url
    output_path = os.path.join(os.path.dirname(__file__), "tmp")
    print(output_path)
    # Проверка на существование и корректность пути
    if not os.path.isdir(output_path):
        os.makedirs(output_path)

    ydl_opts = {
        'outtmpl': os.path.join(output_path, '%(title)s.%(ext)s'),
        'format': 'bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]',
        'noplaylist': True,
        'external_downloader': 'aria2c',
        'external_downloader_args': ['-x', '16', '-j', '16', '-k', '1M'],
        # Параметры для aria2c: 16 потоков и 16 одновременных загрузок
        'retries': 5,
        'fragment_retries': 10,

        'no_warnings': True,
        'quiet': True,
        'socket_timeout': 15,  # Таймаут сокета
        'http_chunk_size': 1048576,  # Размер чанка для HTTP

        'progress_hooks': [
            lambda d: download_progress.update({url: d['_percent_str']}) if d['status'] == 'downloading' else None],
    }

    def ydl_download():
        with YoutubeDL(ydl_opts) as ydl:
            try:
                info = ydl.extract_info(url, download=True)
                filename = ydl.prepare_filename(info)
                logger.info(f"Downloaded video '{filename}'")
                return filename
            except Exception as e:
                logger.error(f"Error downloading video: {str(e)}")
                raise HTTPException(status_code=400, detail=f"Error downloading video: {str(e)}")

    try:
        loop = asyncio.get_event_loop()
        executor = ThreadPoolExecutor(max_workers=1)
        filename = await loop.run_in_executor(executor, ydl_download)
        download_progress.pop(url, None)
        return VideoResponse(filename=filename)
    except HTTPException as e:
        raise e
    except Exception as e:
        logger.error(f"Internal server error: {str(e)}")
        raise HTTPException(status_code=500, detail=f"Internal server error: {str(e)}")


@router.get("/ytdwnld/info", response_model=VideoInfoResponse)
async def get_video_info(request: VideoRequest):
    url = request.url
    ydl_opts = {
        'format': 'bestvideo+bestaudio/best',
        'noplaylist': True,
        'quiet': True,
        'no_warnings': True,
    }

    def ydl_info():
        with YoutubeDL(ydl_opts) as ydl:
            try:
                info = ydl.extract_info(url, download=False)
                return {
                    'title': info.get('title'),
                    'duration': info.get('duration'),
                    'view_count': info.get('view_count')
                }
            except Exception as e:
                logger.error(f"Error fetching video info: {str(e)}")
                raise HTTPException(status_code=400, detail=f"Error fetching video info: {str(e)}")

    try:
        loop = asyncio.get_event_loop()
        executor = ThreadPoolExecutor(max_workers=1)
        info = await loop.run_in_executor(executor, ydl_info)
        return VideoInfoResponse(**info)
    except HTTPException as e:
        raise e
    except Exception as e:
        logger.error(f"Internal server error: {str(e)}")
        raise HTTPException(status_code=500, detail=f"Internal server error: {str(e)}")


@router.get("/ytdwnld/status/{filename}", response_model=DownloadStatusResponse)
async def get_download_status(filename: str):
    file_path = os.path.join("/tmp", filename)
    if os.path.exists(file_path):
        return DownloadStatusResponse(status="completed", progress=100.0)
    elif filename in download_progress:
        return DownloadStatusResponse(status="downloading", progress=float(download_progress[filename].strip('%')))
    else:
        return DownloadStatusResponse(status="not found")


@router.delete("/ytdwnld/delete/{filename}")
async def delete_video(filename: str):
    file_path = os.path.join("/tmp", filename)
    try:
        if os.path.exists(file_path):
            os.remove(file_path)
            logger.info(f"Deleted file '{filename}'")
            return {"status": "deleted"}
        else:
            raise HTTPException(status_code=404, detail="File not found")
    except Exception as e:
        logger.error(f"Error deleting file: {str(e)}")
        raise HTTPException(status_code=500, detail=f"Error deleting file: {str(e)}")


async def fetch(url: str):
    async with aiohttp.ClientSession() as session:
        async with session.get(url) as response:
            if response.status != 200:
                logger.error(f"Error fetching metadata: HTTP {response.status}")
                raise HTTPException(status_code=response.status, detail="Error fetching metadata")
            return await response.text()


@router.get("/ytdwnld/fetch_metadata", response_model=dict)
async def fetch_metadata(request: VideoRequest):
    url = f"https://www.youtube.com/oembed?url={request.url}&format=json"
    try:
        data = await fetch(url)
        return json.loads(data)
    except HTTPException as e:
        raise e
    except Exception as e:
        logger.error(f"Error fetching metadata: {str(e)}")
        raise HTTPException(status_code=400, detail=f"Error fetching metadata: {str(e)}")
