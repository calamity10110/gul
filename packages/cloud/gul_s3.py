"""
GUL S3
S3 Client (Lightweight V4 Signer).

Status: ✅ Implemented
Priority: High
"""

import hmac
import hashlib
import datetime
import urllib.request
import urllib.parse
from typing import Optional, Dict

__version__ = "0.1.0"
__all__ = ['S3Client', 'connect']

class S3Client:
    """
    Lightweight S3 Client (Pure Python, No Boto3)
    """
    
    def __init__(self, region: str, access_key: str, secret_key: str, endpoint: Optional[str] = None):
        self.region = region
        self.access_key = access_key
        self.secret_key = secret_key
        self.endpoint = endpoint or f"https://s3.{region}.amazonaws.com"
        
    def get_object(self, bucket: str, key: str) -> bytes:
        """Download object"""
        url = f"{self.endpoint}/{bucket}/{key}"
        headers = self._sign("GET", url, {}, datetime.datetime.utcnow())
        
        req = urllib.request.Request(url, headers=headers)
        with urllib.request.urlopen(req) as res:
            return res.read()
            
    def put_object(self, bucket: str, key: str, data: bytes):
        """Upload object"""
        url = f"{self.endpoint}/{bucket}/{key}"
        headers = self._sign("PUT", url, {}, datetime.datetime.utcnow(), payload_hash=self._hash(data))
        
        req = urllib.request.Request(url, data=data, headers=headers, method="PUT")
        with urllib.request.urlopen(req) as res:
            return res.read()

    def _sign(self, method: str, url: str, headers: Dict, now: datetime.datetime, payload_hash: str = "UNSIGNED-PAYLOAD") -> Dict:
        # AWS Signature V4 Implementation
        service = "s3"
        host = urllib.parse.urlparse(url).netloc
        date_header = now.strftime('%Y%m%dT%H%M%SZ')
        date_short = now.strftime('%Y%m%d')
        
        canonical_uri = urllib.parse.urlparse(url).path
        canonical_qs = ""
        
        headers['host'] = host
        headers['x-amz-date'] = date_header
        headers['x-amz-content-sha256'] = payload_hash
        
        sorted_headers = sorted(headers.items())
        header_str = "\n".join([f"{k.lower()}:{v.strip()}" for k, v in sorted_headers])
        signed_headers = ";".join([k.lower() for k, v in sorted_headers])
        
        canonical_req = f"{method}\n{canonical_uri}\n{canonical_qs}\n{header_str}\n\n{signed_headers}\n{payload_hash}"
        
        scope = f"{date_short}/{self.region}/{service}/aws4_request"
        string_to_sign = f"AWS4-HMAC-SHA256\n{date_header}\n{scope}\n{self._hash(canonical_req.encode())}"
        
        # Signing Key
        k_date = self._sign_msg(f"AWS4{self.secret_key}".encode(), date_short)
        k_region = self._sign_msg(k_date, self.region)
        k_service = self._sign_msg(k_region, service)
        k_signing = self._sign_msg(k_service, "aws4_request")
        
        signature = hmac.new(k_signing, string_to_sign.encode(), hashlib.sha256).hexdigest()
        
        auth_header = f"AWS4-HMAC-SHA256 Credential={self.access_key}/{scope}, SignedHeaders={signed_headers}, Signature={signature}"
        
        headers['Authorization'] = auth_header
        return headers
        
    def _sign_msg(self, key: bytes, msg: str) -> bytes:
        return hmac.new(key, msg.encode(), hashlib.sha256).digest()
        
    def _hash(self, data: bytes) -> str:
        return hashlib.sha256(data).hexdigest()

def connect(region: str, access_key: str, secret_key: str) -> S3Client:
    return S3Client(region, access_key, secret_key)
