#!/usr/bin/env python3
"""Test that two NEAR nodes can exchange messages using the Proof of Response protocol."""

import sys, time
import pathlib

sys.path.append(str(pathlib.Path(__file__).resolve().parents[2]))

from configured_logger import logger
import pytest
from cluster import init_cluster
from utils import wait_for_blocks

def test_por_echo():
    """Test that nodes can exchange and echo messages using Proof of Response protocol."""
    # Initialize a two-node cluster
    nodes = init_cluster(2, 0, 1, None,
                        {
                            "network": {
                                "por_enabled": True,
                            }
                        })

    # Wait for nodes to start and produce some blocks to ensure they're connected
    wait_for_blocks(nodes[0], count=5)
    logger.info("Nodes started and producing blocks")

    # The nodes should automatically send PoR messages to each other on startup
    # We just need to wait a bit and check their logs
    time.sleep(5)  

    # Search logs for evidence of PoR messages being sent and received
    node0_logs = nodes[0].get_logs()
    node1_logs = nodes[1].get_logs()

    # Look for expected log messages
    assert any("Sending PoR message" in line for line in node0_logs.split('\n')), \
        "Node 0 did not send PoR message"
    assert any("Received PoR message" in line for line in node1_logs.split('\n')), \
        "Node 1 did not receive PoR message"
    assert any("Sending PoR message" in line for line in node1_logs.split('\n')), \
        "Node 1 did not send PoR message"
    assert any("Received PoR message" in line for line in node0_logs.split('\n')), \
        "Node 0 did not receive PoR message"