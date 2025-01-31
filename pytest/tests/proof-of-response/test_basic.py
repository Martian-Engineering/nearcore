#!/usr/bin/env python3
"""Test basic functionality of the proof-of-response protocol between NEAR nodes."""

import sys
import pathlib
import time
import unittest

sys.path.append(str(pathlib.Path(__file__).resolve().parents[2] / 'lib'))

from configured_logger import logger
from cluster import init_cluster, start_cluster
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

class ProofOfResponseTest(unittest.TestCase):
    def test_por_basics(self):
        """Test that two nodes can exchange proof-of-response messages."""
        logger.info("Starting proof-of-response basic test")

        nodes = start_cluster(
            num_nodes=2,
            num_observers=0,
            num_shards=1,
            config=None,
            genesis_config_changes=[
                ("minimum_stake_ratio", [160, 1_000_000]),  # 0.016%
                ("num_block_producer_seats", 2),
                ("num_block_producer_seats_per_shard", [2]),
                ("num_chunk_producer_seats", 2),
                ("num_chunk_validator_seats", 2),
                ("validators", [
                    {"account_id": "test0", "public_key": "ed25519:6DSjZ8mvsRZDvFqFxo8tCKePG96omXW7eVYVSySmDk8e", "amount": "50000000000000000000000000000000"},
                    {"account_id": "test1", "public_key": "ed25519:GkDv7nSMS3xcqA45cpMvFmfV1o4fRF6zYo1JRR6mNqg5", "amount": "50000000000000000000000000000000"}
                ])
            ],
            client_config_changes={
                0: {
                    "network": {
                        "por_enabled": True,
                        "tier1_enable_outbound": True,
                        "tier1_enable_inbound": True,
                        "connect_to_reliable_peers_on_startup": True,
                        "max_num_peers": 40,
                        "minimum_outbound_peers": 5,
                        "ideal_connections_lo": 30,
                        "ideal_connections_hi": 35,
                        "safe_set_size": 20,
                        "ttl_account_id_router": {"secs": 3600, "nanos": 0},
                        "routed_message_ttl": 100,
                        "peer_stats_period": {"secs": 5, "nanos": 0},
                        "handshake_timeout": {"secs": 5, "nanos": 0},
                        "skip_sync_wait": True,
                        "peer_recent_time_window": {"secs": 600, "nanos": 0},
                        "monitor_peers_max_period": {"secs": 100, "nanos": 0},
                        "ban_window": {"secs": 180, "nanos": 0},
                        "blacklist_threshold": 10,
                        "tcp_keepalive_interval": {"secs": 30, "nanos": 0},
                        "peer_expiration_duration": {"secs": 7200, "nanos": 0},
                        "minimum_broadcast_peers": 3,
                        "public_addrs": ["ed25519:6DSjZ8mvsRZDvFqFxo8tCKePG96omXW7eVYVSySmDk8e@127.0.0.1:24577"],
                        "allow_private_ip_in_public_addrs": True,
                        "message_filter": {
                            "max_messages_per_minute": 1000,
                            "max_messages_per_peer_per_minute": 100,
                            "max_messages_per_peer_per_second": 10,
                            "max_message_size": 1024 * 1024 * 10  # 10MB
                        },
                        "message_handler": {
                            "block_message": True,
                            "block_approval": True,
                            "chunk_message": True,
                            "transaction_message": True,
                            "state_sync_message": True,
                            "network_info_message": True
                        },
                        "message_processor": {
                            "block_message": True,
                            "block_approval": True,
                            "chunk_message": True,
                            "transaction_message": True,
                            "state_sync_message": True,
                            "network_info_message": True
                        },
                        "message_routing": {
                            "block_message": True,
                            "block_approval": True,
                            "chunk_message": True,
                            "transaction_message": True,
                            "state_sync_message": True,
                            "network_info_message": True
                        },
                        "routing": {
                            "ttl": 100,
                            "route_back_ttl": 100,
                            "peer_forwarding": True,
                            "route_back": True
                        },
                        "peer_manager": {
                            "boot_nodes": [],
                            "whitelist_nodes": [],
                            "min_num_peers": 1,
                            "max_num_peers": 2,
                            "ideal_connections": 2,
                            "minimum_outbound_peers": 1,
                            "safe_set_size": 2,
                            "archival_peer_connections_lower_bound": 1,
                            "handshake_timeout": 5,
                            "skip_sync_wait": True,
                            "peer_states_cache_size": 1000,
                            "peer_states_cache_ttl": 6000,
                            "connect_only_to_boot_nodes": False,
                            "block_broadcast": True,
                            "block_request": True,
                            "block_headers_request": True
                        }
                    },
                    "consensus": {
                        "min_num_peers": 1,
                        "block_production_tracking_delay": {
                            "secs": 0,
                            "nanos": 50000000
                        },
                        "min_block_production_delay": {
                            "secs": 0,
                            "nanos": 100000000
                        },
                        "max_block_production_delay": {
                            "secs": 0,
                            "nanos": 400000000
                        },
                        "max_block_wait_delay": {
                            "secs": 1,
                            "nanos": 0
                        },
                        "produce_empty_blocks": True,
                        "block_fetch_horizon": 50,
                        "block_header_fetch_horizon": 50,
                        "catchup_step_period": {
                            "secs": 0,
                            "nanos": 100000000
                        },
                        "chunk_request_retry_period": {
                            "secs": 0,
                            "nanos": 400000000
                        },
                        "doomslug_step_period": {
                            "secs": 0,
                            "nanos": 100000000
                        },
                        "doomslug_threshold_mode": "NoApprovals"
                    }
                },
                1: {
                    "network": {
                        "por_enabled": True,
                        "tier1_enable_outbound": True,
                        "tier1_enable_inbound": True,
                        "connect_to_reliable_peers_on_startup": True,
                        "max_num_peers": 40,
                        "minimum_outbound_peers": 5,
                        "ideal_connections_lo": 30,
                        "ideal_connections_hi": 35,
                        "safe_set_size": 20,
                        "ttl_account_id_router": {"secs": 3600, "nanos": 0},
                        "routed_message_ttl": 100,
                        "peer_stats_period": {"secs": 5, "nanos": 0},
                        "handshake_timeout": {"secs": 5, "nanos": 0},
                        "skip_sync_wait": True,
                        "peer_recent_time_window": {"secs": 600, "nanos": 0},
                        "monitor_peers_max_period": {"secs": 100, "nanos": 0},
                        "ban_window": {"secs": 180, "nanos": 0},
                        "blacklist_threshold": 10,
                        "tcp_keepalive_interval": {"secs": 30, "nanos": 0},
                        "peer_expiration_duration": {"secs": 7200, "nanos": 0},
                        "minimum_broadcast_peers": 3,
                        "public_addrs": ["ed25519:GkDv7nSMS3xcqA45cpMvFmfV1o4fRF6zYo1JRR6mNqg5@127.0.0.1:24578"],
                        "allow_private_ip_in_public_addrs": True,
                        "message_filter": {
                            "max_messages_per_minute": 1000,
                            "max_messages_per_peer_per_minute": 100,
                            "max_messages_per_peer_per_second": 10,
                            "max_message_size": 1024 * 1024 * 10  # 10MB
                        },
                        "message_handler": {
                            "block_message": True,
                            "block_approval": True,
                            "chunk_message": True,
                            "transaction_message": True,
                            "state_sync_message": True,
                            "network_info_message": True
                        },
                        "message_processor": {
                            "block_message": True,
                            "block_approval": True,
                            "chunk_message": True,
                            "transaction_message": True,
                            "state_sync_message": True,
                            "network_info_message": True
                        },
                        "message_routing": {
                            "block_message": True,
                            "block_approval": True,
                            "chunk_message": True,
                            "transaction_message": True,
                            "state_sync_message": True,
                            "network_info_message": True
                        },
                        "routing": {
                            "ttl": 100,
                            "route_back_ttl": 100,
                            "peer_forwarding": True,
                            "route_back": True
                        },
                        "peer_manager": {
                            "boot_nodes": [],
                            "whitelist_nodes": [],
                            "min_num_peers": 1,
                            "max_num_peers": 2,
                            "ideal_connections": 2,
                            "minimum_outbound_peers": 1,
                            "safe_set_size": 2,
                            "archival_peer_connections_lower_bound": 1,
                            "handshake_timeout": 5,
                            "skip_sync_wait": True,
                            "peer_states_cache_size": 1000,
                            "peer_states_cache_ttl": 6000,
                            "connect_only_to_boot_nodes": False,
                            "block_broadcast": True,
                            "block_request": True,
                            "block_headers_request": True
                        }
                    },
                    "consensus": {
                        "min_num_peers": 1,
                        "block_production_tracking_delay": {
                            "secs": 0,
                            "nanos": 50000000
                        },
                        "min_block_production_delay": {
                            "secs": 0,
                            "nanos": 100000000
                        },
                        "max_block_production_delay": {
                            "secs": 0,
                            "nanos": 400000000
                        },
                        "max_block_wait_delay": {
                            "secs": 1,
                            "nanos": 0
                        },
                        "produce_empty_blocks": True,
                        "block_fetch_horizon": 50,
                        "block_header_fetch_horizon": 50,
                        "catchup_step_period": {
                            "secs": 0,
                            "nanos": 100000000
                        },
                        "chunk_request_retry_period": {
                            "secs": 0,
                            "nanos": 400000000
                        },
                        "doomslug_step_period": {
                            "secs": 0,
                            "nanos": 100000000
                        },
                        "doomslug_threshold_mode": "NoApprovals"
                    }
                },
            },
        )

        # Wait for nodes to start and produce some blocks 
        wait_for_blocks(nodes[0], count=5, timeout=60)
        logger.info("Nodes started and producing blocks")

        # Check logs for evidence of PoR messages being sent and received
        # The nodes are hardcoded to send "hello" to their peers on startup
        self.assertTrue(wait_for_message(nodes[0], "Sending PoR message"),
            "Node 0 did not send PoR message")
        self.assertTrue(wait_for_message(nodes[1], "Received PoR message"),
            "Node 1 did not receive PoR message")
        self.assertTrue(wait_for_message(nodes[1], "Sending PoR message"),
            "Node 1 did not echo message back")
        self.assertTrue(wait_for_message(nodes[0], "Received PoR message"),
            "Node 0 did not receive echo response")

        # Clean up
        for node in nodes:
            node.kill()

        logger.info("Proof-of-response basic test completed successfully")

if __name__ == '__main__':
    unittest.main()