#!/usr/bin/env python3
"""Test basic functionality of the proof-of-response protocol between NEAR nodes."""

import asyncio
import os
import sys
import time
import pathlib

sys.path.append(str(pathlib.Path(__file__).resolve().parents[2]))
import pytest

from configured_logger import logger
from cluster import init_cluster
from utils import wait_for_blocks

TIMEOUT_SEC = 5

def wait_for_message(node, expected_message, timeout=TIMEOUT_SEC):
    """Wait for the expected message in a node's logs."""
    start = time.time()
    while time.time() - start < timeout:
        logs = node.get_logs()
        if expected_message in logs:
            return True
        time.sleep(0.1)
    return False

def test_por_basics():
    """Test that two nodes can exchange proof-of-response messages."""
    # Initialize a two-node cluster with the PoR protocol enabled
    nodes = init_cluster(
        2, 0, 1, None,
        {
            "network": {
                "por_enabled": True
            }
        }
    )

    # Wait for nodes to start and produce some blocks 
    wait_for_blocks(nodes[0], count=5)
    logger.info("Nodes started and producing blocks")

    # Check logs for evidence of PoR messages being sent and received
    # The nodes are hardcoded to send "hello" to their peers on startup
    assert wait_for_message(nodes[0], "Sending PoR message"), \
        "Node 0 did not send PoR message"
    assert wait_for_message(nodes[1], "Received PoR message"), \
        "Node 1 did not receive PoR message"
    assert wait_for_message(nodes[1], "Sending PoR message"), \
        "Node 1 did not echo message back"
    assert wait_for_message(nodes[0], "Received PoR message"), \
        "Node 0 did not receive echo response"